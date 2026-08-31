#!/usr/bin/env python3
"""Corpus hygiene probe — what fraction of each KB's chunks are unusable text?

Written after sampling for ontology authoring showed the RFCs corpus is
character-spaced ("L I S T t y p i c a l l y") and Climate's chunks are largely
bibliographies, funding acknowledgements and download watermarks. No ontology
fixes either. This measures how much of each corpus is not prose about its
subject, because that bounds what any extraction vocabulary can achieve.
"""
import json, os, re, sqlite3, sys, datetime, yaml
CONFIG = os.path.expanduser("~/.kwaainet/config.yaml")
HERE = os.path.dirname(os.path.abspath(__file__))

def char_spaced(t):
    """PDF extraction that split every character: 'T h e  q u i c k'."""
    toks = t.split()
    if len(toks) < 20:
        return False
    return sum(1 for w in toks if len(w) == 1) / len(toks) > 0.5

REF = re.compile(r"(et al\.|doi:|DOI |https?://|\bpp\.\s*\d|\bvol\.\s*\d)", re.I)
def reference_like(t):
    """Bibliography / citation-list text rather than body prose."""
    hits = len(REF.findall(t))
    years = len(re.findall(r"\(\s*(?:19|20)\d\d\s*[a-z]?\s*\)", t))
    return (hits + years) >= 4 and len(t) < 4000

TAG = re.compile(r"<(?:div|span|pre|li|ul|a|p|table|tr|td|br|h[1-6])\b[^>]*>", re.I)
def markup_heavy(t):
    """Raw HTML/XML left in the chunk — Poems and OSMDocs are full of it."""
    return len(TAG.findall(t)) >= 3

VTT = re.compile(r"\d\d:\d\d:\d\d[.,]\d\d\d\s*-->")
def transcript_scaffold(t):
    """WebVTT cue numbers and timestamps inline in the text (Meetings)."""
    return len(VTT.findall(t)) >= 2

def self_duplicated(t):
    """Chunk contains the same passage twice — seen across Poems, Meetings,
    OSMDocs. Halves the useful content and double-counts every mention."""
    t = " ".join(t.split())
    if len(t) < 200:
        return False
    h = len(t) // 2
    return t[:h][:120] in t[h:] or t[h:][:120] in t[:h]

BOILER = re.compile(
    r"(all rights reserved|copyright ©|downloaded from|this page intentionally"
    r"|prepublication copy|printed in the united states|isbn|library of congress"
    r"|guest \(guest\) ip)", re.I)

def probe(kb, cfg):
    e = cfg[kb]
    d = e.get("rag_data_dir") or os.path.expanduser(f"~/.kwaainet/rag/{kb}")
    db = os.path.join(d, f"{e['tenant_id']}.db")
    if not os.path.exists(db):
        return None
    c = sqlite3.connect(f"file:{db}?mode=ro", uri=True)
    try:
        texts = [json.loads(v)["text"] for (v,) in
                 c.execute("SELECT CAST(value AS TEXT) FROM chunks")]
    finally:
        c.close()
    if not texts:
        return None
    n = len(texts)
    # First matching category wins, so counts partition the corpus.
    cats = {"char_spaced": 0, "markup": 0, "transcript": 0,
            "reference_like": 0, "boilerplate": 0, "duplicated": 0}
    clean = 0
    for t in texts:
        if char_spaced(t):        cats["char_spaced"] += 1
        elif markup_heavy(t):     cats["markup"] += 1
        elif transcript_scaffold(t): cats["transcript"] += 1
        elif reference_like(t):   cats["reference_like"] += 1
        elif BOILER.search(t):    cats["boilerplate"] += 1
        elif self_duplicated(t):  cats["duplicated"] += 1
        else:                     clean += 1
    return {"kb": kb, "chunks": n, **cats,
            "clean": clean, "clean_rate": round(clean / n, 3)}

def main():
    cfg = yaml.safe_load(open(CONFIG))["rag_kbs"]
    kbs = sys.argv[1:] or sorted(cfg)
    rows = [r for r in (probe(k, cfg) for k in kbs) if r]
    rows.sort(key=lambda r: r["clean_rate"])
    stamp = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
    out = os.path.join(HERE, "results", f"corpus_hygiene_{stamp}.md")
    with open(out, "w") as f:
        f.write(f"# Corpus hygiene — {datetime.datetime.now(datetime.timezone.utc):%Y-%m-%dT%H:%M:%SZ}\n\n")
        f.write("What fraction of each KB is text an extractor can actually work with.\n")
        f.write("`char-spaced` = PDF extraction split every character. `ref-like` =\n")
        f.write("bibliography/citation lists. `boiler` = copyright, watermarks, ISBNs.\n\n")
        f.write("| kb | chunks | char-spaced | markup | transcript | ref-like | boiler | self-dup | clean % |\n")
        f.write("|---|---|---|---|---|---|---|---|---|\n")
        for r in rows:
            f.write(f"| {r['kb']} | {r['chunks']} | {r['char_spaced']} | {r['markup']} | "
                    f"{r['transcript']} | {r['reference_like']} | {r['boilerplate']} | "
                    f"{r['duplicated']} | **{r['clean_rate']*100:.1f}%** |\n")
    with open(out.replace(".md", ".json"), "w") as f:
        json.dump(rows, f, indent=2)
    print(out)

if __name__ == "__main__":
    main()
